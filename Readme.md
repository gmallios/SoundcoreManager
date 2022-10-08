# soundcore-rs

## Get info and control soundcore earbuds with the power of rust.

### Working 
- ANC Get/Set


### TODO: 
- Case Battery decode
- Earbud Battery decode
- EQ Get/Set
- Move project to lib format
- Make it linux compatible


# Notes
```
Hardcoded windows api usage atm.
```

Example reponse(hex array) 
```
[9, FF, 0, 0, 1, 1, 1, 61, 0, 1, 1, 3, 4, 1, 0, 0, 0, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, 78, FF, FF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 63, 1, 54, 1, 66, 1, 54, 0, 1, 0, 0, 0, 2, 1, 0, 0, 1, 0, 0, 0, 0, 6E]
```