import "./App.css";

import { commands, Card, CardQuery } from "./bindings.ts"

async function testDB() {
  commands.initDb();
  let card: Card = {id:null, name: "TestCard", decks: [] };
  card = await commands.createOrUpdateCard(card);
  
  let query: CardQuery = {id:null,name:"TestCard", potential_decks:[{id:null,name:"first deck",cards:[]}]};
  let queryResult = await commands.getCards(query);
  console.log(queryResult);

  
}

function App() {

  return (
    <button onClick={async () => { await testDB(); }}>
      Test db
    </button>
  );
}

export default App;
