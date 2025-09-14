
app/_layout.tsx
```typescriptreact
import { Stack } from "expo-router";

export default function RootLayout() {
  return <Stack />;
}
```
_______________________________________________________________________________

app/index.tsx
```typescriptreact
import { Text, View } from "react-native";

export default function Index() {
  return (
    <View
      style={{
        flex: 1,
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Text>Edit app/index.tsx to edit this screen.</Text>
    </View>
  );
}
```
_______________________________________________________________________________
