import ClobStats from "./components/ClobStats";
import CreateOrderForm from "./components/CreateOrderForm";

function App() {
  return (
    
    <div>
      <div className="p-8">
      <h1 className="text-xl font-bold mb-4">CLOB Frontend</h1>
      <CreateOrderForm />
    </div>
      <h1>CLOB Dashboard</h1>
      <ClobStats />
    </div>
  );
}

export default App;
