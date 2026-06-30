# syscfg
- This is a tool to help save & load the configuration of a linux system and its apps. 

## Usage
- `syscfg mksnap [-o /path/to/snapshot]`: Creates a snapshot directory with a default configuration. If `-o` is not supplied, the snapshot path will default to `~/.config/syscfg/`

## Config 
- The configuration for this looks like the following 
````
```json
{
	"AppConfig": [],
	"Font": "*",
	"Package": [],
}
```
