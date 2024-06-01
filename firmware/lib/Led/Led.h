#include <Arduino.h>
#include <Adafruit_NeoPixel.h>

class Led {
    private:
        int _pin;  // Пин, к которому подключена лента
        int _count;  // Количество светодиодов в ленте

        unsigned long lastUpdate = 0;  // Время последнего обновления анимации
        const int animationSteps = 50;  // Количество шагов в анимации
        const int animationDelay = 20;  // Задержка в миллисекундах между шагами
        int currentStep = 0;  // Текущий шаг анимации
        bool animating = false;  // Флаг анимации
        int startColor[3];  // Начальный цвет
        int endColor[3];  // Конечный цвет

    public:
        Adafruit_NeoPixel strip;  // Объект управления лентой
        int brightness[5] = {100, 100, 100, 100, 100};  // Яркость
        Led(int pin, int count) 
            : _pin(pin), _count(count), strip(count, pin, NEO_RGB + NEO_KHZ800) {}

        void begin() {
            strip.begin();
            strip.show();  // Инициализируем ленту
        }

        void setPixelsInRange(int startIndex, int endIndex, int color) {
            for (int index = startIndex; index <= endIndex; index++) {
                strip.setPixelColor(index, color);
            }
            strip.show();
        }

        void setBrightness(int startIndex, int endIndex, int value) {
            for (int i = startIndex; i < endIndex + 1; i++)
            {
                brightness[i] = value;
            }
        }

        void startAnimationFading() {
            for (int i = 0; i < _count; i++) {
                if (brightness[i] > 0) {
                    uint32_t color = strip.getPixelColor(i);
                    uint8_t r = ((color & 0x00FF0000) >> 16) * brightness[i] / 100;
                    uint8_t g = ((color & 0x0000FF00) >> 8) * brightness[i] / 100;
                    uint8_t b = (color & 0x000000FF) * brightness[i] / 100;

                    strip.setPixelColor(i, strip.Color(r, g, b));
                    brightness[i]--;
                    strip.show();
                }
            }
        }
};