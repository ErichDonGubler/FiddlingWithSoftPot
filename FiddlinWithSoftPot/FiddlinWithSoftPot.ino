const int SOFT_POT_PIN = A0; // Pin connected to softpot wiper
const int NUMBER_OF_NOTES_PLUS_ONE = 13;
// This is plus one because I don't have a linear softpot,
// and the extra space taken by 13 makes 12 easier to press.
int lastSoftPotPosition = 0;

void setup() 
{
  Serial.begin(9600);
  pinMode(SOFT_POT_PIN, INPUT);
}

void loop() 
{
  // Read in the soft pot's ADC value
  int softPotADC = analogRead(SOFT_POT_PIN);
  // Map the 0-1023 value to 0-NUMBER_OF_NOTES_PLUS_ONE
  int softPotPosition = map(softPotADC, 0, 1023, 0, NUMBER_OF_NOTES_PLUS_ONE);

  // I check softPotPosition is not 13 here because a slice is used in the rust code.
  if(softPotPosition != lastSoftPotPosition && softPotPosition != 13){
    if (softPotPosition < 10) {
      Serial.print(0);
    }
    Serial.print(softPotPosition);
    lastSoftPotPosition = softPotPosition;
  }
}
