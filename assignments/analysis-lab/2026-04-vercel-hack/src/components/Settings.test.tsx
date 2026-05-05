import { test, expect, describe, mock } from "bun:test";
import { render } from "@testing-library/react";
import { Settings } from "./Settings";

describe("Settings", () => {
  test("renders token input field", () => {
    const { container } = render(<Settings />);
    const inputs = container.querySelectorAll("input");
    expect(inputs.length).toBeGreaterThan(0);
  });

  test("renders save button", () => {
    const { container } = render(<Settings />);
    expect(container.textContent).toMatch(/kaydet|save/i);
  });

  test("renders without crashing", () => {
    const { container } = render(<Settings />);
    expect(container).toBeTruthy();
  });
});
