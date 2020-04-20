const int SOFT_POT_PIN = A0; // Pin connected to softpot wiper
const int GRAPH_LENGTH = 13; // Length of line graph
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
  // Map the 0-1023 value to 0-GRAPH_LENGTH
  int softPotPosition = map(softPotADC, 0, 1023, 0, GRAPH_LENGTH);

  if(softPotPosition != lastSoftPotPosition){
    if (softPotPosition < 10) {
      Serial.print(0);
    }
    Serial.print(softPotPosition);
    lastSoftPotPosition = softPotPosition;
  }
}
