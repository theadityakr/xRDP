import React, { useState } from "react";
import Form from "../components/Form.tsx";
import "../App.css";

const Home: React.FC = () => {
  const [greetMsg, setGreetMsg] = useState("");

  return (
    <div className="container">
      <div className="flex-column">
      <h1>Welcome to RDP!</h1>
      <Form onGreet={setGreetMsg} />
      </div>
    </div>
  );
};

export default Home;