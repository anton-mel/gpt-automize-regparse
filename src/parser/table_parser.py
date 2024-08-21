from parser.openai_client import client
from openai import OpenAIError
import os


registers_summary_path = './registers_summary.csv'

def write_header_if_not_exists():
    """Write the CSV header if the file does not already exist."""
    if not os.path.isfile(registers_summary_path):
        with open(registers_summary_path, mode='w', newline='', encoding='utf-8') as file:
            file.write("Offset, Abbreviation, Name, Block, RW, Reset, Page\n")

def extract_table_data(pdf_text):
    """ Prompt for extracting data in CSV format """

    prompt = (
        "Given the following PDF text, identify and extract the 'Registers Summary PF — BAR 0' table. "
        "Please structure the extracted data in the following format:\n\n"
        "Offset, Abbreviation, Name, Block, RW, Reset, Page.\n\n"
        "Extract all the rows and provide them in a clean, CSV format.\n\n"
        "BAR 0 has multiple subsections, so skip this text if needed and continue with the same format.\n"
        "Continue extracting data unless you encounter information that does not follow the specified format.\n\n"
        "Assume that currently all the data provided that follows the format should be extracted."
        "Return only the extracted data, with no additional comments or explanations.\n"
        "Your output should be able to completely be copied to a CSV file.\n\n"
        "The header line is not needed in the extracted data; it is already present in the file.\n\n"
        f"{pdf_text}"
    )

    try:
        response = client.chat.completions.create(
            model="gpt-4o-mini",
            messages=[
                {"role": "system", "content": "You are an expert data extractor."},
                {"role": "user", "content": prompt}
            ],
        )

        table_data = response.choices[0].message.content.strip()
        table_data = table_data.replace("```", "").strip()
        
        # Append the data to the CSV file
        with open(registers_summary_path, mode='a', newline='', encoding='utf-8') as file:
            file.write(table_data + '\n')

        print(f"CSV data successfully written to {registers_summary_path}")

    except OpenAIError as e:
        raise RuntimeError(f"An error occurred while making a request to OpenAI: {e}")
