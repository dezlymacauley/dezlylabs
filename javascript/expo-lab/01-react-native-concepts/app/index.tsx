// This imports the built-in `Text` and `View` components from React Native
// Think of `<View></View>` as the equivalent of the HTML tag `<div></div>`
 
import { Text, ButtonProps, View, Image, ScrollView, FlatList, Button, Alert } from "react-native";

export default function Index() {
  return (
    <View
       style={{
         flex: 1,
         justifyContent: "center",
         alignItems: "center",
       }}
    >
      <Text>hello</Text>
      <Button 
        onPress={() => Alert.alert("alert title", "alert message")} 
        title="Delete account" 
        color={"red"} 
      />
    </View>
  );
}
