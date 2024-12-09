#ifndef PRINT_HH
#define PRINT_HH

#include <cstdint>

namespace Print {

        enum class Color : uint8_t {
                Black = 0,
                Blue = 1,
                Green = 2,
                Cyan = 3,
                Red = 4,
                Magenta = 5,
                Brown = 6,
                LightGray = 7,
                DarkGray = 8,
                LightBlue = 9,
                LightGreen = 10,
                LightCyan = 11,
                LightRed = 12,
                LightMagenta = 13,
                Yellow = 14,
                White = 15
        };

        // default print white on black
        void print(const char* str);

        // print with specified color
        void print(const char* str, Color foreground, Color background);

        // clear the screen
        void clear();

        // move cursor to specific position
        void setCursor(int x, int y);
}

#endif // PRINT_HH
