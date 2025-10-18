// You can assign a `string` value to an enum 
// to make it easier to understand when used with `console.log()`
enum PageTheme {
  Dark = "Dark",   
  Light = "Light",
  System = "System"
}

const themeSelected: PageTheme = PageTheme.Dark;

console.log(`themeSelected: ${themeSelected}`);
// themeSelected: Dark
