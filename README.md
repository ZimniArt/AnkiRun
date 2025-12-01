**Language Immersion Text Translator**

This program creates a text where words from one language are gradually  replaced by words from another language based on their frequency in the text. The result is text where words are mixed, which should aid in picking up meaning from the context, and bridge the gap where it's difficult to find texts that are at the exact level of your skill at the language. The most common words are translated first, while less common words are translated later, which makes sure that you will encounter the word again and again in the text and learn it in the context. 

You can fine tune how difficult the new text will be. You can translate anywhere from 1 most popular word in the text to the whole text (though translating the full text might take a while).

**Example of resulted text. en -> ja**

"_One theme throughout これ book は それ ゲーム dissections depend の上 あなたの
目標 として a dissector. もし あなた’re ただ trying to 得る a rough understanding の
シャベル 騎士, understanding それ the プレーヤー’s アクション は Run, Jump, そして_"


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
