/**
 * @jest-environment jsdom
 */
import "@testing-library/jest-dom";
import "@testing-library/jest-dom/extend-expect";

import { render } from "@testing-library/svelte";

import Item from "./Item.svelte";

test("does it test", () => {
  const { getByText } = render(Item, {
    item: { summary: "this is an item", start: { dateTime: new Date() } },
  });
  const text = getByText(/this is an item/);
  expect(text).toBeInTheDocument();
});
