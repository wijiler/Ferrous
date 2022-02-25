# Ferriclang notes

## Sectoring
each semicolon and opening bracket = new "sector" if function skip

Code Example:

```ferric
// entrypoint 
!!main(*args here*) { // new sector
} // eos
```

But sectors can have subsectors(as seen with most functions)

```ferric
!!main(*args here*) {
  print("Hiya there"); // new subsector
}
```

this would be seen as

```ferric
entrypoint:args = argsSector[ 0x0:NULL ]:new = {:does =
	subsector:new = keyword:id=print:args = argsSector[ 0x0:String = Hiya there]:does = 
		keywordprint = args
end:}
--//subsector
----------------- // eos
```


