import json
import pandas as pd

with open('br.json') as f:
    d = json.load(f)

verses = []
hebrew_alphabet = ["א", "ב", "ג", "ד", "ה", "ו", "ז", "ח", "ט", "י", "כ", "ל", "מ", "נ", "ס", "ע", "פ", "צ", "ק", "ר", "ש", "ת"]

column_list = ["verse"] + hebrew_alphabet

df = pd.DataFrame(columns=column_list)


for chapter_number, chapter in enumerate(d["text"], 1):
	for verse_number, verse in enumerate(chapter, 1):
		shablon = {"verse_number": "", "verse": "", "א": 0, "ב": 0, "ג": 0, "ד": 0, "ה": 0, "ו": 0, "ז": 0, "ח": 0, "ט": 0, "י": 0, "כ": 0, "ל": 0, "מ": 0, "נ": 0, "ס": 0, "ע": 0, "פ": 0, "צ": 0, "ק": 0, "ר": 0, "ש": 0, "ת": 0}

		shablon["verse_number"] = str(chapter_number) + ":" + str(verse_number)
		shablon["verse"] = verse

		num_consonants = len(verse) - verse.count(' ')
		verse = verse.replace("ך", "כ")
		verse = verse.replace("ם", "מ")
		verse = verse.replace("ן", "נ")
		verse = verse.replace("ף", "פ")
		verse = verse.replace("ץ", "צ")

		#print(chapter_number, ":", verse_number, " ", verse)
		for letter in hebrew_alphabet:
			shablon[letter] = verse.count(letter)/num_consonants
		verses.append(shablon)
			
print(json.dumps(verses, ensure_ascii=False))