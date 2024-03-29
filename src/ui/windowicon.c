#ifndef WINDOW_ICON_C_
#define WINDOW_ICON_C_

#include "raylib.h"
#include "../const.h"

INCBIN(window_icon, "assets/icon/icon-big.png");

void load_window_icon(void) {
#ifndef __APPLE__
    Image window_icon = LoadImageFromMemory(
        ".png", window_icon_data, window_icon_size
    );
    SetWindowIcon(window_icon);
    UnloadImage(window_icon);
#endif
}

#endif