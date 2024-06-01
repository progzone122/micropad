#include <Adafruit_NeoPixel.h>
#include <AsyncStream.h>
#include <EncButton.h>
#include <SimpleTimer.h>
#include <Keyboard.h>
#include <StringUtils.h>
#include <EEManager.h>

#include <Led.h>
// pins
#define BUTTONS_PINS { 6, 7, 8 }
#define PR_PIN A0 // POTENTIOMETER

// RGB LED
#define LED_PIN 10
#define LED_COUNT 5

const int buttons_pins[] = BUTTONS_PINS;

// settings
struct Settings
{
  // buttons [3]
  // color RGB [3]
  int colors[3][3] = {{0, 86, 222}, {0, 86, 222}, {0, 86, 222}};
  int color_inactive[3] = {0, 144, 255};
  int fade_animation_speed = 50;
  char b1_hotkeys[50] = "";
  char b2_hotkeys[50] = "";
  char b3_hotkeys[50] = "";
};
Settings settings;

AsyncStream<50> serial(&Serial, '\n', 50);
EEManager memory(settings);
Led led(LED_PIN, LED_COUNT);

int lightLed;

// timers
SimpleTimer timer1;

// Buttons
Button button0(buttons_pins[0], INPUT_PULLUP, LOW);
Button button1(buttons_pins[1], INPUT_PULLUP, LOW);
Button button2(buttons_pins[2], INPUT_PULLUP, LOW);

void setup()
{
  Serial.begin(9600);
 
  Keyboard.begin();

  led.begin();
  led.setPixelsInRange(0, 4, led.strip.Color(settings.color_inactive[0], settings.color_inactive[1], settings.color_inactive[2]));

  memory.begin(0, 'b');
}

void loop()
{
  if (memory.tick()) Serial.println("Updated!");

  timer1.setInterval(settings.fade_animation_speed, []() {
    led.startAnimationFading();
  });

  // Если пришли на микроконтроллер данные по Serial
  if (serial.available()) {
    String command;
    su::Text message(serial.buf);
    int button_index = 0;

    memset(settings.b1_hotkeys, 0, sizeof(settings.b1_hotkeys));
    memset(settings.b2_hotkeys, 0, sizeof(settings.b2_hotkeys));
    memset(settings.b3_hotkeys, 0, sizeof(settings.b3_hotkeys));

    for (su::TextParser message_row(message, ';'); message_row.parse();) {
      if (message_row.index() == 0) {
        command = message_row.toString();
      }
      if (message_row.index() > 0) {
        if (command == "wh") {
          for (su::TextParser col(message_row, ','); col.parse();) {
            if (col.index() == 0) {
              button_index = col.toInt32();
            }
            if (col.index() > 1) {
              // Serial.println(col);
              switch (button_index) {
                case 1:
                  strcat(settings.b1_hotkeys, ",");
                  char buf1[100];
                  col.toStr(buf1);
                  strcat(settings.b1_hotkeys, buf1);
                  break;
                case 2:
                  strcat(settings.b2_hotkeys, ",");
                  char buf2[100];
                  col.toStr(buf2);
                  strcat(settings.b2_hotkeys, buf2);
                  break;
                case 3:
                  strcat(settings.b3_hotkeys, ",");
                  char buf3[100];
                  col.toStr(buf3);
                  strcat(settings.b3_hotkeys, buf3);
                  break;
                default:
                  break;
              }
            }
              
          }
        }
      }
    }
    memory.update();
  }

  button0.tick();
  button1.tick();
  button2.tick();

  if(button0.release() || button1.release()) {
    Keyboard.releaseAll();
  }

  if(button0.pressing()) {
    led.setPixelsInRange(3, 4, led.strip.Color(settings.colors[0][0], settings.colors[0][1], settings.colors[0][2]));
    led.setBrightness(3, 4, 100);

    String value[10] = {};
    int value_index = 0;
    for (su::TextParser col(settings.b1_hotkeys, ','); col.parse();) {
      // Serial.println(col);
        value[value_index] = col.toString();
        value_index++;
    }

    for (int i = 0; i < value_index; i++) {
      if (value[i].startsWith("0x")) {
        // Преобразуем строку с шестнадцатеричным числом в число
        int keyCode = strtol(value[i].c_str(), nullptr, 16);
        Keyboard.press(keyCode);
      } else if (value[i].length() == 1 && isPrintable(value[i][0])) {
        // Если это один символ, отправляем его как строку
        Keyboard.press(value[i][0]);
      } else {
        // Иначе, предполагаем, что это код клавиши
        Keyboard.press(value[i].toInt());
      }
    }
  }
  if(button1.pressing()) {
    led.setPixelsInRange(1, 2, led.strip.Color(settings.colors[0][0], settings.colors[0][1], settings.colors[0][2]));
    led.setBrightness(1, 2, 100);

    String value[10] = {};
    int value_index = 0;
    for (su::TextParser col(settings.b2_hotkeys, ','); col.parse();) {
      value[value_index] = col.str();
      value_index++;
    }

    for (int i = 0; i < value_index; i++) {
      if (value[i].startsWith("0x")) {
        // Преобразуем строку с шестнадцатеричным числом в число
        int keyCode = strtol(value[i].c_str(), nullptr, 16);
        Keyboard.press(keyCode);
      } else if (value[i].length() == 1 && isPrintable(value[i][0])) {
        // Если это один символ, отправляем его как строку
        Keyboard.press(value[i][0]);
      } else {
        // Иначе, предполагаем, что это код клавиши
        Keyboard.press(value[i].toInt());
      }
    }
  }
  if(button2.pressing()) {
    led.setPixelsInRange(0, 0, led.strip.Color(settings.colors[0][0], settings.colors[0][1], settings.colors[0][2]));
    led.setBrightness(0, 0, 100);

    String value[10] = {};
    int value_index = 0;
    for (su::TextParser col(settings.b3_hotkeys, ','); col.parse();) {
      value[value_index] = col.str();
      value_index++;
    }

    for (int i = 0; i < value_index; i++) {
      if (value[i].startsWith("0x")) {
        // Преобразуем строку с шестнадцатеричным числом в число
        int keyCode = strtol(value[i].c_str(), nullptr, 16);
        Keyboard.press(keyCode);
      } else if (value[i].length() == 1 && isPrintable(value[i][0])) {
        // Если это один символ, отправляем его как строку
        Keyboard.press(value[i][0]);
      } else {
        // Иначе, предполагаем, что это код клавиши
        Keyboard.press(value[i].toInt());
      }
    }
  }

  lightLed = round(analogRead(PR_PIN) / (1023 / 255));

  timer1.run();
}