import { config } from "../../src/config";
import { toSlackMarkdown } from "../../src/handlers/slack";

beforeAll(() => {
  config.groups = [
    {
      gitlab: {
        id: 100,
        name: "dev",
      },
      slack: {
        id: "A01234567",
        name: "dev",
      },
    },
  ];
  config.users = [
    {
      gitlab: {
        id: 11,
        name: "lechuck",
      },
      slack: {
        id: "U0123456789",
        name: "LechuckRoh",
      },
    },
  ];
});

describe("markdown", () => {
  it("mentions slack user", () => {
    const text = "Hello @lechuck";
    const expected = "Hello <@U0123456789>";
    const actual = toSlackMarkdown(text);
    expect(actual).toEqual(expected);
  });

  it("mentions slack group", () => {
    const text = "@dev";
    const expected = "<!subteam^A01234567>";
    const actual = toSlackMarkdown(text);
    expect(actual).toEqual(expected);
  });

  it("converts link", () => {
    const text = "[GitHub](https://github.com)";
    const expected = "<https://github.com|GitHub>";
    const actual = toSlackMarkdown(text);
    expect(actual).toEqual(expected);
  });

  it("converts image link", () => {
    const projectUrl = "https://gitlab.com/foo/bar";
    const imageFilename = "Screen_Shot";
    const imagePath =
      "/uploads/ee4efa9e8aba1bc2a5b5337fb39bde1d/Screen_Shot.png";
    const text = `![${imageFilename}](${imagePath})`;

    const expected = `<${projectUrl}${imagePath}|${imageFilename}>`;
    const actual = toSlackMarkdown(text, projectUrl);
    expect(actual).toEqual(expected);
  });
});
