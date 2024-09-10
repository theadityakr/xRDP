import React, { useState } from "react";
import Form from "../components/form.tsx";
import "../App.css";

const Home: React.FC = () => {
  const [greetMsg, setGreetMsg] = useState("");

  return (
    <div className="container">
      <h1>Welcome to RDP!</h1>
      <Form onGreet={setGreetMsg} />
      <p>{greetMsg}</p>
    </div>
  );
};

export default Home;