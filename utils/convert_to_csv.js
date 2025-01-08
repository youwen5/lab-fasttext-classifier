const fs = require("fs");
const path = require("path");

// Input and output file paths
const inputFilePath = path.join(__dirname, "data/gpt-dataset.txt");
const outputFilePath = path.join(__dirname, "data/dataset.csv");

// Function to convert labeled data to CSV format
function convertToCSV(inputFile, outputFile) {
  try {
    // Read the input file
    const data = fs.readFileSync(inputFile, "utf8");

    // Use regex to extract labels and text
    const matches = [...data.matchAll(/__(label__[^ ]+) (.+)/g)];

    // Create CSV content
    let csvContent = "Label,Text\n";
    matches.forEach((match) => {
      const [, label, text] = match;
      csvContent += `${label},"${text.replace(/"/g, '""')}"\n`;
    });

    // Write to the output CSV file
    fs.writeFileSync(outputFile, csvContent);
    console.log(`CSV file has been saved at ${outputFile}`);
  } catch (error) {
    console.error("Error:", error);
  }
}

// Run the function
convertToCSV(inputFilePath, outputFilePath);
