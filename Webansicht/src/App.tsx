import { useState } from "react";

function App() {
  const [name, setName] = useState("");
  const [age, setAge] = useState("");
  const [greeting, setGreeting] = useState("");
  const [message, setMessage] = useState("");

  async function sendGreet() {
    const response = await fetch("http://localhost:3000/api/greet", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        name,
        age,
      }),
    });

    const data = await response.json();

    setGreeting(data.greeting);
  }

  async function getMessage() {
    const response = await fetch("http://localhost:3000/api/hello", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      }
    });

    const data = await response.json();
    setMessage(data.message);
  }

  return (
    <div style={{ padding: "2rem" }}>
      <h1>React + Axum</h1>

      <input
        value={name}
        onChange={(e) => setName(e.target.value)}
        placeholder="Dein Name"
      />

      <input
        value={age}
        onChange={(e) => setAge(e.target.value)}
        placeholder="Dein Alter"
      />

      <button onClick={sendGreet}>
        Senden
      </button>

      <button onClick={getMessage}>
        Nachricht Holen
      </button>

      <h2>{greeting}</h2>

      <h2>{message}</h2>
    </div>
  );
}

export default App;