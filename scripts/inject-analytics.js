import fs from "node:fs/promises";
import * as cheerio from "cheerio";

const targetFile = "./docs/index.html";

const analyticsScript = `
<script
  src="https://beamanalytics.b-cdn.net/beam.min.js"
  data-token="bf73f66b-6d2b-446b-88bd-d7feda0d8f94"
  async
></script>
`;

async function injectAnalyticsScript() {
	try {
		// Check if the target file exists
		await fs.access(targetFile);

		// Load the existing HTML
		const data = await fs.readFile(targetFile, "utf8");
		const $ = cheerio.load(data);

		// Append the script to the <head> tag
		$("head").append(analyticsScript);

		// Write the modified HTML back to the file
		await fs.writeFile(targetFile, $.html(), "utf8");

		console.log("HTML updated successfully.");
	} catch (error) {
		console.error("Error updating HTML:", error);
	}
}

injectAnalyticsScript();
