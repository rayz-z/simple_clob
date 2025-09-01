import { useEffect, useState } from "react";

interface Time {
  secs_since_epoch: number;
  nanos_since_epoch: number;
}

interface Order {
  buy_order: boolean;
  price: number;
  quantity: number;
  id: number;
  time_created: Time;
}

interface Transaction {
  price: number;
  quantity: number;
  time: Time;
}

interface OrderBook {
  total_orders: number;
  buy_orders: Record<string, Order[]>;  // keys are prices ("5", "6", …)
  sell_orders: Record<string, Order[]>;
  transactions: Transaction[];
}


export default function ClobStats() {
  const [data, setData] = useState<OrderBook | null>(null);

  useEffect(() => {
    fetch("http://localhost:3000/clob-stats")
      .then((res) => res.json())
      .then((json) => setData(json));
  }, []);

  return (
    <pre>{data ? JSON.stringify(data, null, 2) : "Loading..."}</pre>
  );
}