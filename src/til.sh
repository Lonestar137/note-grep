NOTEDIR=/home/jonesgc/Documents/TIL/
alias note="ranger ~/Documents/ranger-notes/snippets"
alias til="ranger ${NOTEDIR}"
alias ntil='FILENAME=$(date +%m-%d-%y) && if [ ! -s "${NOTEDIR}${FILENAME}.md" ]; then echo "$
{FILENAME}:
---" >> "${NOTEDIR}${FILENAME}.md"; fi && $EDITOR "${NOTEDIR}${FILENAME}.md"'

