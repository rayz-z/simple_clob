export interface CreateOrder {
    buy_order: boolean;
    price: number;
    quantity: number;
}

export async function createOrder(order: CreateOrder) {
    const res = await fetch("http://localhost:3000/orders", {method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify(order)});

    if (!res.ok) {
        throw new Error(`Failed to create order: ${res.statusText}`)
    }

    return res.json(); //why does json need to be called as function and what are the other attributes of res
}

