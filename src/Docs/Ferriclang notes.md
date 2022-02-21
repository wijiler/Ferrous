# Ferriclang notes

## Sectoring
each semicolon and opening bracket = new "sector" if function skip

Code Example:
```
// entrypoint 
!!main(*args here*) { // new sector
} // eos
```

But sectors can have subsectors(as seen with most functions)
```
!!main(*args here*) {
  print("Hiya there"); // new subsector
}
```
this would be seen as
```
entrypoint(args:*nothing here*)does:
keyword(name:print,type:function)does:
  io:print
--//subsector
----------------- // eos
```


