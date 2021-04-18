import React from "react";
import dynamic from "next/dynamic";
import { message, Typography, Tag, Divider, Button, Input, Space } from "antd";
import Container from "../components/layout/Container";

const Invoice = {
  name: "Joyce",
  invoice_number: 3,
  date: "19.04.2021",
  products: [
    {
      qty: 1,
      name: "MacBook Pro",
      price: 400,
    },
    {
      qty: 4,
      name: "iPhone SE",
      price: 399,
    },
  ],
};

const RustComponent = dynamic({
  loader: async () => {
    let alertText = "";

    const handleButtonClick = (wasm, alertText) => {
      wasm.greet(alertText);
    };

    try {
      const wasm = await import("webassembly");
      const result = wasm.add(5, 7);

      const invoice = wasm.invoice_information(Invoice);
      return (props) => (
        <>
          <Typography.Title level={4}>
            Aufruf einer Rust Funktion:
          </Typography.Title>
          <Typography.Paragraph>
            <Tag color="blue">wasm.add(5, 7);</Tag> {result}
          </Typography.Paragraph>
          <Divider />
          <Space direction="vertical">
            <Typography.Title level={4}>
              Aufruf einer Rust Funktion im onClick-Event:
            </Typography.Title>
            <Input
              placeholder="Text für Alert-Funktion"
              onChange={(e) => {
                alertText = e.target.value;
              }}
            ></Input>
            <Button
              type="primary"
              onClick={() => handleButtonClick(wasm, alertText)}
            >
              Rufe Alert-Funktion auf
            </Button>
          </Space>
          <Divider />
          <Typography.Title level={4}>
            Beispiel - Rechnungsoperationen:
          </Typography.Title>
          <Typography.Paragraph>
            <Tag color="blue">Pointer</Tag> {invoice?.ptr}
          </Typography.Paragraph>
        </>
      );
    } catch (error) {
      message.warning("Rust Komponenten nicht verfügbar");
    }
  },
});

const webassembly = () => {
  return (
    <Container>
      <Typography.Title level={1}>
        WebAssembly: Rust Komponenten in React
      </Typography.Title>
      <RustComponent />
    </Container>
  );
};

export default webassembly;
