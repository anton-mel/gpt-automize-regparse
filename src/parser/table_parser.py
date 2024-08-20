from openai import OpenAI, OpenAIError
import os

api_key = os.getenv('OPENAI_API_KEY')

if api_key is None:
    raise EnvironmentError(
        "The OPENAI_API_KEY environment variable is not set."
)

client = OpenAI(
    api_key=api_key,
)

def extract_table_data(pdf_text):
    # Challenges: cannot extract large amount of data
    # needs better promp to extract only the tables needed

    prompt = (
        "Given the following PDF text, find the device register summary table, "
        "extract the data, and present it in a structured format:\n\n"
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

        table_data = response.choices[0].message.content

        return table_data

    except OpenAIError as e:
        raise RuntimeError(f"An error occurred while making a request to OpenAI: {e}")
