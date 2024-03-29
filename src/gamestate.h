#ifndef SELECTEDGAME_H_
#define SELECTEDGAME_H_

#include "raylib.h"
#include "const.h"
#include <stdlib.h>
#include <assert.h>

typedef enum SelectedGame {
    Selected_None = 0,
    Selected_GOL,
    Selected_DVD,
    Selected_PONG,
} SelectedGame;

typedef enum GameState {
    GameState_Running = 0,
    GameState_Paused,
    GameState_Help,
} GameState;

// global state
typedef struct GlobalState {
    bool show_fps;
    int screen_w;
    int screen_h;
    Vector2 mouse_pos;
    Vector2 mouse_delta;
    bool left_mouse_down;
    bool right_mouse_down;
    float mouse_wheel_move;
} GlobalState;

static GlobalState global_state = {0};

/// Update the global state, called once at the beginning of each frame
void update_global_state(void) {
    global_state.screen_w = GetScreenWidth();
    global_state.screen_h = GetScreenHeight();
    global_state.left_mouse_down = IsMouseButtonDown(MOUSE_BUTTON_LEFT);
    global_state.right_mouse_down = IsMouseButtonDown(MOUSE_BUTTON_RIGHT);
    global_state.mouse_pos = GetMousePosition();
    global_state.mouse_delta = GetMouseDelta();
    global_state.mouse_wheel_move = GetMouseWheelMove();
}

#define GLOBAL_TEXT_BUF_SIZE (256 + 1)
static char global_text_buf[GLOBAL_TEXT_BUF_SIZE];

/// global snprintf to global_text_buf
#define g_sprintf(FMT, ...) snprintf(global_text_buf, GLOBAL_TEXT_BUF_SIZE, FMT, ##__VA_ARGS__)

static const char* SELECTED_TITLES[] = {
    [Selected_None] = "MultiSim - menu",
    [Selected_GOL] = "MultiSim - Game of Life",
    [Selected_DVD] = "MultiSim - DvD bouncy",
    [Selected_PONG] = "MultiSim - Pong",
};

#define selected_get_window_title(S) (SELECTED_TITLES[S])

#endif