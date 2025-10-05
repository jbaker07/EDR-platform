import crypto from "node:crypto";
import { createClient, RedisClientType } from "redis";

const DEFAULT_TTL_SECONDS = Number(process.env.CGRP_TTL_SECONDS ?? 3600);
const NOVELTY_EXPIRY_SECONDS = Number(process.env.NOVELTY_TTL_SECONDS ?? 86_400);
const NOVELTY_SATURATION = Number(process.env.NOVELTY_SATURATION ?? 10);
const REDIS_URL = process.env.REDIS_URL;

let redisClient: RedisClientType | null = null;
let redisInitPromise: Promise<RedisClientType | null> | null = null;

async function ensureRedis(): Promise<RedisClientType | null> {
  if (!REDIS_URL) return null;
  if (redisClient) return redisClient;
  if (!redisInitPromise) {
    redisInitPromise = (async () => {
      try {
        const client = createClient({ url: REDIS_URL });
        client.on("error", (err) => {
          console.warn("redis connection error", err);
        });
        await client.connect();
        redisClient = client;
        return client;
      } catch (err) {
        console.warn("redis unavailable, falling back to memory", err);
        return null;
      }
    })();
  }
  return redisInitPromise;
}

interface MemoryRecord {
  hash: string;
  expires: number;
}

class MemoryDedupe {
  private readonly store = new Map<string, MemoryRecord>();

  async seen(key: string, hash: string, ttlSeconds: number): Promise<boolean> {
    const now = Date.now();
    const existing = this.store.get(key);
    if (existing && existing.expires > now) {
      if (existing.hash === hash) {
        return true;
      }
    }
    this.store.set(key, { hash, expires: now + ttlSeconds * 1000 });
    return false;
  }
}

class MemoryNovelty {
  private readonly store = new Map<string, Map<string, { count: number; expires: number }>>();

  score(endpointKey: string, featureHash: string, ttlSeconds: number): number {
    const now = Date.now();
    const bucket = this.store.get(endpointKey) ?? new Map();
    if (!this.store.has(endpointKey)) this.store.set(endpointKey, bucket);
    const entry = bucket.get(featureHash);
    if (entry && entry.expires > now) {
      entry.count += 1;
      entry.expires = now + ttlSeconds * 1000;
      return Math.max(0, 1 - Math.min(1, entry.count / NOVELTY_SATURATION));
    }
    bucket.set(featureHash, { count: 1, expires: now + ttlSeconds * 1000 });
    return 1;
  }
}

const memoryDedupe = new MemoryDedupe();
const memoryNovelty = new MemoryNovelty();

export function sha1(input: string): string {
  return crypto.createHash("sha1").update(input).digest("hex");
}

export async function isDuplicate(
  dedupeKey: string,
  payloadHash: string,
  ttlSeconds: number = DEFAULT_TTL_SECONDS
): Promise<boolean> {
  const client = await ensureRedis();
  if (!client) {
    return memoryDedupe.seen(dedupeKey, payloadHash, ttlSeconds);
  }
  const existing = await client.get(dedupeKey);
  if (existing === payloadHash) {
    return true;
  }
  await client.set(dedupeKey, payloadHash, { EX: ttlSeconds });
  return false;
}

export async function noveltyScore(
  endpointKey: string,
  featureHash: string,
  ttlSeconds: number = NOVELTY_EXPIRY_SECONDS
): Promise<number> {
  const client = await ensureRedis();
  if (!client) {
    return memoryNovelty.score(endpointKey, featureHash, ttlSeconds);
  }
  const redisKey = `novelty:${endpointKey}`;
  const newCount = await client.hIncrBy(redisKey, featureHash, 1);
  await client.expire(redisKey, ttlSeconds);
  const ratio = Math.min(1, newCount / NOVELTY_SATURATION);
  return Math.max(0, 1 - ratio);
}
