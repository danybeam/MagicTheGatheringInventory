import "./App.css";

import { commands, Card } from "./bindings.ts"

async function testDB() {
  commands.initDb();
  let card: Card = {id:null, name: "TestCard", decks: [] };
  card = await commands.createOrUpdateCard(card);
  await new Promise(resolve => setTimeout(resolve, 10000));
  card.name = "something else";
  card = await commands.createOrUpdateCard(card);
}

function App() {

  return (
    <button onClick={async () => { await testDB(); }}>
      Test db
    </button>
  );
}

export default App;
