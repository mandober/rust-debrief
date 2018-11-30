# Screen output in DOS

- DOS Interrupt 21h
- BIOS Interrupt 10h
- Direct video access

When an application program needs to write characters on the screen in text mode, it can choose between three types of video output:
1. MS-DOS level access: `int 21h` to write text to video display.
2. BIOS level access: output using BIOS services i.e. `int 10h` function, which  executes faster than `int 21h` and permits the control of text color.
3. Direct video access: characters are moved directly to video RAM (screen buffer), resulting in instantaneous execution.

Screen Buffer:
* begins at `B8 00 00 00` (rows and columns).
* Number of columns is usually 80
* Number of rows is usually 25 or 40;
* Making the 80x25 or 80x40 grid
* Each screen line needs 160B: 80*2B (1B for char itself, 1B for attribute).

## VIDEO Programming with `int 10h`

There are 2 general types of video modes: text and graphics mode.

Video Text Mode
* Fonts:  
  Characters are generated from a memory-resident table of character fonts. Originally, the table was in ROM, but later versions of the BIOS permitted programmers to rewrite the character tables at run time, making it possible to create custom fonts in text mode.
* Video Text Pages:   
  Text mode video memory is divided into multiple separate video pages, each with the ability to hold a full screen of text. Programs can display one page while writing text to other hidden pages, and they can rapidly flip back and forward between pages. The default video page is Page 0. This text page feature became obsolete. 
* Attributes:    
  each screen character is assigned an attribute byte that controls both the color of the character (foreground) and the screen color behind the character (background). Each position on the video display holds a single character, along with its own attribute (color). The attribute is stored in a separate byte, following the character in memory.
* Blinking:    
  The video controller perfmorms blinking by reversing the foreground and background colors of a character at a predetermined rate. By default, when a PC boots into MS-DOS mode, blinking is enabled. It is possible to turn blinking off using a video BIOS function. Blinking is off by default in MS-DOS prompt under Windows.


INT 10h Video Functions:
* `ah` register contains the function number
* `00h`: Set video mode
  * Video Text modes:
    - 1, 40x25, 16 colors
    - 3, 80x25, 16 colors
    - 14h, 132x25, 16 colors
    - (3 more modes with 2 colors)
  * Video Graphics modes:
    - 10h, 640x350, 16 colors
    - 13h, 320x200, 256 colors
    - 6ah, 800x600, 16 colors
    - (9 modes total)
* `01h`: Set cursor lines
* 02h: Set cursor position
* 03h: Get cursor position and size
* 06h: Scroll window up
* 07h: Scroll window down
* 08h: Read character and attribute
* `09h`: Write character and attribute
* `0Ah`: Write character
* 10h (AL = 03h): Toggle blinking/intensity bit
* 0Fh: Get video mode



