import { assert, expect, test } from "vitest";
import Item from "../../components/agenda/Item.svelte";

test("basic functionality", () => {
  const host = document.createElement("div");
  document.body.appendChild(host);
  const instance = new Item({
    target: host,
    props: {
      item: {
        summary: "do a thing",
        start: { dateTime: new Date(2022, 1, 1, 1) },
      },
    },
  });
  expect(host.innerHTML).toContain("do a thing");
});
