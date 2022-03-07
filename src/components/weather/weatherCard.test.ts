/**
 * @jest-environment jsdom
 */
import { render } from "@testing-library/svelte";
import WeatherCard from "./weatherCard.svelte";

test("does it test", () => {
  const { getByText } = render(WeatherCard, {
    temp: 20,
    humidity: 90,
    at: new Date(),
  });
  const temp = getByText("Temperature: 20");
  expect(temp).toBeInTheDocument();
});
