import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import dotenv from "dotenv";
import { correlate } from "../../src/correlation/engine";
import { fanoutNormalizedAlert } from "../../src/ingest/fanout";
import { VendorObservation } from "../../src/types/alerts";

dotenv.config({ path: process.env.ENV_FILE || ".env" });

async function loadFixtures(): Promise<VendorObservation[]> {
  const fixturePath = process.env.FIXTURE_PATH ?? path.join(process.cwd(), "dev/fixtures/correlated-demo.json");
  const raw = await fs.promises.readFile(fixturePath, "utf8");
  return JSON.parse(raw);
}

async function main() {
  const observations = await loadFixtures();
  for (const obs of observations) {
    const alerts = await correlate(obs);
    for (const alert of alerts) {
      const { results } = await fanoutNormalizedAlert(alert);
      // eslint-disable-next-line no-console
      console.log(`fanout for ${alert.correlation_group_id} →`, results);
    }
  }
}

main().catch((err) => {
  console.error(err);
  process.exitCode = 1;
});
