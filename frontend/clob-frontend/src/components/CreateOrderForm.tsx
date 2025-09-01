import { useState } from "react";
import { createOrder } from "../api";

interface Response {
    responseCode: number,
    id: number,
}

export default function CreateOrderForm() {
    const [price, setPrice] = useState<number>(0);
    const [quantity, setQuantity] = useState<number>(0);
    const [buyOrder, setBuyOrder] = useState(true);
    const [response, setResponse] = useState<Response | null>(null);

    async function handleSubmit(e:React.FormEvent) {
        e.preventDefault();
        try {
            const result = await createOrder({ buy_order: buyOrder, price, quantity });
            setResponse(result);
        } catch (err) {
            console.error(err);
        }
    }

    return (
        <div className="p-4">
      <form onSubmit={handleSubmit} className="space-y-2">
        <label>
          Type:
          <select
            value={buyOrder ? "buy" : "sell"}
            onChange={(e) => setBuyOrder(e.target.value === "buy")}
          >
            <option value="buy">Buy</option>
            <option value="sell">Sell</option>
          </select>
        </label>

        <label>
          Price:
          <input
            type="number"
            value={price}
            onChange={(e) => setPrice(Number(e.target.value))}
          />
        </label>

        <label>
          Quantity:
          <input
            type="number"
            value={quantity}
            onChange={(e) => setQuantity(Number(e.target.value))}
          />
        </label>

        <button type="submit">Create Order</button>
      </form>

      {response && (
        <pre className="mt-4 bg-gray-100 p-2 rounded">
          {JSON.stringify(response, null, 2)}
        </pre>
      )}
    </div>
    );
}