**Language Immersion Text Translator**

This program creates a text where words from one language are replaced by words from another language based on their frequency in the text. The most common words are translated first, while less common words are translated later.

I came up with this idea to aid language learning through immersion. The goal is to help readers pick up words from context rather than relying solely on dictionaries.

  ⚠ Note: The program uses an online translator, so some translations may occasionally be unusual. It is not context-sensitive.

I’ve added support for 20 languages and tried to make the user experience as smooth as possible.

**How to use:**

  1.  Select a text file (currently only .txt files are supported).
  
  2. Select the text language. There is an auto-detect option, but it can give unpredictable results, so it’s recommended to choose the correct language manually
  
  3. Select the target language you want to translate the text into.
  
  4. Set the number of popular words to translate.
  
  5. Suggested starting range: 30–100 words.The more words you choose, the longer the translation will take.

The translated file will be saved in the same folder as the original file with _translated appended to the filename.

_I hope this tool will help someone! I personally wanted a tool like this for two years, and now that I’m learning Rust, I decided to make it my first project._
