# Morse Library changelog
[0.2.1]
* Update Readme.md
* Add Description for MorseUnit enum
  
[0.2.0]
* *:
    - add library documentation with examples
* Morse struct
    - add .to_text() method, that convert Morse Code back to text
    - rename from_str() to from_int_text()
    - rename from_bin() to from_int_bin()
* Sound struct
    - create TSound trait with default implementation
    - imlement TSound for Sound struct
* MorseUnit enum
    - delete redundant enum imlementation

[0.1.0]
* First release