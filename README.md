# Stamp

![Rust](https://github.com/JakeStanger/stamp/workflows/Pipeline/badge.svg)

A command line tool for creating one or more files from templates, written in Rust.

Stamp was born out of a lack of tooling that filled this middle ground:

- IntelliJ supports creating files from templates, but only one file at a time.
- Tools like Cookiecutter and Yeoman are slow and far too large for quick usage.

## Installation

You will require rust and cargo installed.

```bash
cargo install stampr
```

Annoyingly the crate name `stamp` was already taken.

You might want to `alias=stamp=stampr`.

## Usage

### Command Line Usage

Command line help can be viewed using `stampr -h` or `stampr --help` for more detail.

You can also use `stampr help <command>`.

#### List

`stampr list` will show a list of installed templates. 
Passing the `-v` flag will also show the path of each template on disk.

#### Run

`stampr run <template>` will render a template and create the files.

By default, files are written relative to your current working directory. 
To change this, use `-o` or `--out`. 
If the path does not exist, it will be created for you.

Arguments for a template can be specified using the `-c` or `--context` flag, 
and provided in `key=value` pairs. 
Any arguments not provided will prompt you for input.
This must be the last flag provided.

Example:

```bash
stampr run test --out /tmp/stamp-test -c greeting=Hello subject=World
```

### Creating Templates

#### Locations

Templates can be stored globally or inside a directory. When looking for templates, Stamp will:
- Look for a `.stamp/templates` folder inside the current directory
- Recurse up each parent directory, looking for the same folder
- Check the global directory

If multiple templates of the same name exist, the first one found is used. 
This means it is possible to include templates in your repository, 
and to override global templates by creating one of the same name inside a folder.

Global templates are stored inside the current user's configuration folder.

- Linux: `~/.config/stamp/templates`
- OSX: `$USER/Library/Preferences/stamp`
- Windows: `%appdata%\stamp\templates`

#### Files

To create a template, simply create a directory with the template name in a templates folder.

Each file and folder inside that directory is then templated. 
The file name and contents are rendered using [Handlebars](https://handlebarsjs.com/guide/).
A number of helpers are available, and the list can be found [here](https://github.com/davidB/handlebars_misc_helpers/tree/v0.9.0).

Variables used in templates are automatically detected and turned into arguments at runtime.

#### Example

One such use-case might be to scaffold out a single TypeScript React component, 
including a file for the props and another for the stylesheet.

The end goal is to end up with this structure:
```
testComponent
├── ITestComponentProps.ts
├── TestComponent.module.scss
└── TestComponent.tsx
```

To create the template, create a folder called `tsx-component` in a Stamp template folder.

Inside the template folder, create a new folder to house the component files.
Name it `{{to_camel_case name}}`. 
This means the directory will always take the name argument and ensure it is in camelCase.

Inside that directory, create three files:

- `{{to_pascal_case name}}.tsx`
- `I{{to_pascal_case name}}Props.ts`
- `{{to_pascal_case name}}.module.scss`

The first file houses imports for the other two, as well as a basic functional component:

```tsx
import * as React from 'react';
import I{{to_pascal_case name}}Props from './I{{to_pascal_case name}}Props';
import styles from '{{to_pascal_case name}}.module.scss';

const {{to_pascal_case name}}: React.FC<I{{to_pascal_case name}}Props> = () => {
  return <span />;
};

export default {{to_pascal_case name}};
```

The second contains a blank interface:

```ts
interface I{{to_pascal_case name}}Props {

}

export default I{{to_pascal_case name}}Props;
```

And the third can remain empty.

Now run your template:
```bash
stampr run tsx-component -c name=testComponent
```

You should see a directory created called `testComponent` with 3 files inside.

> Note it would be possible to avoid using the helpers above and just use `{{name}}` 
>if you do not want causing to automatically be adjusted. 