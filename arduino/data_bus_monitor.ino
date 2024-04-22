const char DATA[] = {};

void setup() {
  pinMode(3, OUTPUT); 

  for (int n = 0; n < 8; n += 1) {
    pinMode(n + 4, INPUT);
  }

  Serial.begin(9600);
}

void loop() {
  digitalWrite(3, HIGH);
  delay(250);
  digitalWrite(3, LOW);
  delay(250);

  unsigned int address = 0;
  for (int n = 0; n < 8; n += 1 ) {
    int bit = digitalRead(n + 4);
    Serial.print(bit);
    address = (address << 1) + bit;
  }

  Serial.print(" ");
  Serial.print(address, HEX);

  Serial.print("\n");
}