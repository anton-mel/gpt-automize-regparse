from utils.pdf_utils import extract_text_from_pdf
from parser.openai_client import client
from openai import OpenAIError
import csv
import os


registers_summary_path = './registers_summary.csv'
registers_info_folder = './registers_info'

def snake_case(s):
    """Convert a string to snake_case."""
    return s.replace(" ", "_").replace("/", "_").replace("-", "_").lower()

def extract_and_save_register_data(pdf_file_path, offset, name, page_number):
    """Extract detailed information for a specific register and save it to a CSV file."""
    output_file = os.path.join(registers_info_folder, f"{snake_case(name)}.csv")
    
    # Step 1: Extract the relevant page text from the PDF
    pdf_text = extract_text_from_pdf(pdf_file_path, page_start=page_number, page_end=page_number)

    # Step 2: Prepare the prompt for extracting detailed register information
    prompt = (
        f"Given the following PDF text provided below, extract the detailed information for the register '{name}' located at offset '{offset}'. "
        "Please structure the extracted data in the following CSV format:\n\n"
        "Field,Bit(s),Init Val,Reserved\n\n"
        "The last column should indicate whether the bits are reserved or not, using 'True' for reserved bits and 'False' for non-reserved bits analized from Description. "
        "Do NOT include this header in the output since it is already present in the file."
        "Provide the data without any spaces after commas, aiming for a clean CSV format.\n\n"
        "If any data in the columns contains a comma (`,`), replace it with a hyphen (`-`) to maintain CSV integrity.\n\n"
        "Return only the extracted data, with no additional comments or explanations. Your output should be ready to write directly to a CSV file.\n\n"
        "To summarize, your output should consist solely of the extracted data: NO additional explanation, NO header describing the data.\n\n"
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

        detailed_data = response.choices[0].message.content.strip()
        detailed_data = detailed_data.replace("```", "").strip()

        # Step 3: Write the extracted data to the register's CSV file
        with open(output_file, mode='w', newline='', encoding='utf-8') as file:
            file.write(detailed_data + '\n')

        print(f"Detailed data for {name} successfully written to {output_file}")

    except OpenAIError as e:
        raise RuntimeError(f"An error occurred while making a request to OpenAI: {e}")

def process_registers_summary(pdf_file_path):
    """Process each row in the registers summary CSV file and extract detailed information."""
    if not os.path.exists(registers_info_folder):
        os.makedirs(registers_info_folder)

    with open(registers_summary_path, mode='r', newline='', encoding='utf-8') as file:
        reader = csv.DictReader(file)
        for row in reader:
            offset = row['Offset']
            name = row['Name']
            page_number_str = row['Page']
            
            if page_number_str and page_number_str.strip().isdigit():
                page_number = int(page_number_str.strip())
                print(page_number)
                extract_and_save_register_data(pdf_file_path, offset, name, page_number)
            else:
                print(f"Skipping {name} at offset {offset} due to missing or invalid page number.")
