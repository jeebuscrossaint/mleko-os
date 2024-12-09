#include "print.hh"

namespace Print {
    namespace {
        const int VGA_WIDTH = 80;
        const int VGA_HEIGHT = 25;
        volatile uint16_t* const VIDEO_MEMORY = (uint16_t*)0xB8000;

        int cursor_x = 0;
        int cursor_y = 0;

        uint8_t makeColor(Color foreground, Color background) {
            return static_cast<uint8_t>(foreground) | (static_cast<uint8_t>(background) << 4);
        }

        uint16_t makeVgaEntry(char c, uint8_t color) {
            return static_cast<uint16_t>(c) | (static_cast<uint16_t>(color) << 8);
        }
    }

    void print(const char* str) {
        print(str, Color::White, Color::Black);
    }

    void print(const char* str, Color foreground, Color background) {
        uint8_t color = makeColor(foreground, background);

        for(int i = 0; str[i] != '\0'; i++) {
            if(str[i] == '\n') {
                cursor_x = 0;
                cursor_y++;
                if(cursor_y >= VGA_HEIGHT) cursor_y = 0;
                continue;
            }

            const int index = cursor_y * VGA_WIDTH + cursor_x;
            VIDEO_MEMORY[index] = makeVgaEntry(str[i], color);

            cursor_x++;
            if(cursor_x >= VGA_WIDTH) {
                cursor_x = 0;
                cursor_y++;
                if(cursor_y >= VGA_HEIGHT) cursor_y = 0;
            }
        }
    }

    void clear() {
        uint8_t color = makeColor(Color::White, Color::Black);
        for(int y = 0; y < VGA_HEIGHT; y++) {
            for(int x = 0; x < VGA_WIDTH; x++) {
                const int index = y * VGA_WIDTH + x;
                VIDEO_MEMORY[index] = makeVgaEntry(' ', color);
            }
        }
        cursor_x = 0;
        cursor_y = 0;
    }

    void setCursor(int x, int y) {
        if(x >= 0 && x < VGA_WIDTH && y >= 0 && y < VGA_HEIGHT) {
            cursor_x = x;
            cursor_y = y;
        }
    }
}
