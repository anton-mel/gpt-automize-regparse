import os
from openai import OpenAI

api_key = os.getenv('OPENAI_API_KEY')

if api_key is None:
    raise EnvironmentError("The OPENAI_API_KEY environment variable is not set.")

client = OpenAI(api_key=api_key)
