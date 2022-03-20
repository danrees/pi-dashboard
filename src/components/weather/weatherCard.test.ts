/**
 * @jest-environment jsdom
 */
import { render, within } from "@testing-library/svelte";
import WeatherCard from "./weatherCard.svelte";

test("does it test", () => {
  const { getByText } = render(WeatherCard, {
    temp: 20,
    humidity: 90,
    at: new Date(),
    feelsLike: 20,
    tempMin: 15,
    tempMax: 25,
  });
  const temp = getByText(/^Temperature:/);

  expect(temp).toBeInTheDocument();
  expect(temp).toHaveTextContent("Temperature: 20°C");
});
