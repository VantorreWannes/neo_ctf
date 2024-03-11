
# Open the file and read the contents
with open('green.bin', 'rb') as file:
    raw_bytes = file.read()

print(raw_bytes.decode())
