# Asynmov: A Python Package for Generating Synthetic Data for Historical Fiction World Building

Asynmov is a powerful and flexible Python package designed to assist writers, game developers, and storytellers in creating rich, immersive historical fiction worlds by generating synthetic data based on real-world historical events, cultures, and societies. With Asynmov, you can effortlessly generate a wide range of realistic and consistent data points to bring your fictional worlds to life.

## Features

### Entities

- **Character Generation**: Create detailed character profiles with names, ages, backgrounds, occupations, education, and more, based on historical demographics and social structures.

### Events

- **Core Life (Vital) Events**:
  - Birth events with historically accurate naming conventions and family structures.
  - Death events reflecting historical life expectancies and mortality rates.
  - Marriage events based on historical customs, dowries, and arranged marriages.

- **Vocation Events**:
  - Occupation events based on historical professions, guilds, and apprenticeships.
  - Education events reflecting historical literacy rates, universities, and scholarly pursuits.
  - Military service events based on historical conscription, wars, and battles.

- **Avocation Events**:
  - Religious events based on historical beliefs, practices, and pilgrimages.
  - Cultural events reflecting historical arts, music, literature, and festivities.
  - Sports and leisure events based on historical pastimes, games, and hunting.

- **Relocation Events**:
  - Migration events based on historical patterns of settlement, colonization, and diaspora.
  - Urbanization events reflecting historical growth of cities and rural-to-urban migration.
  - Exploration events based on historical voyages, expeditions, and trade routes.

- **Upheaval Events**:
  - War and conflict events based on historical battles, sieges, and invasions.
  - Natural disaster events reflecting historical earthquakes, floods, and famines.
  - Political events based on historical revolutions, coups, and regime changes.

### Relationship Development

- **Social Network Generation**: Create historically accurate social networks based on family ties, class structures, and political alliances.
- **Relationship Dynamics**: Model the evolution of relationships over time based on historical social norms, arranged marriages, and political maneuvering.

### World Building Utilities

- **Historical Timeline Generation**: Construct plausible historical events, timelines, and chronologies for your fictional world based on real-world historical patterns and causality.
- **Cultural and Social Systems**: Develop intricate cultural and social systems, such as languages, religions, governments, and economies, based on historical anthropology and sociology.
- **Technology and Innovation**: Generate historically accurate technology levels, inventions, and scientific discoveries based on real-world technological progression.
- **Data Export**: Export the generated data in various formats, such as JSON and CSV, for easy integration into other tools and platforms.

## Feature Pipeline

- **Geographic Generation**: Generate convincing historical maps, landscapes, and geographic features based on real-world terrain, climate, and ecosystems.
- **Alternate History Scenarios**: Explore "what if" scenarios by altering key historical events and generating plausible alternate timelines and outcomes.
- **Customizable Templates**: Customize and extend the generated data using user-defined templates, rule sets, and historical databases.

## Installation

To install Asynmov, simply use pip:

```
pip install asynmov
```

## Usage

Here's a quick example of how to use Asynmov to generate a historically accurate character profile:

```python
from asynmov import HistoricalCharacterGenerator

generator = HistoricalCharacterGenerator(era="medieval", region="europe")
character = generator.generate_character()

print(character)
```

Output:
```
{
  "name": "Adelard FitzWalter",
  "age": 35,
  "gender": "Male",
  "occupation": "Knight",
  "education": "Martial training, Heraldry",
  "background": "Born into a noble family, Adelard was trained from a young age in the art of combat and chivalry. He has served his liege lord faithfully in various military campaigns and is known for his bravery and skill with a sword."
}
```

For more detailed usage instructions and examples, please refer to the [documentation](https://github.com/yourusername/asynmov/docs).

## Contributing

We welcome contributions from the community! If you'd like to contribute to Asynmov, please read our [contributing guidelines](https://github.com/yourusername/asynmov/CONTRIBUTING.md) and submit a pull request.

## License

Asynmov is open-source software licensed under the [Apache License 2.0](https://github.com/yourusername/asynmov/LICENSE).

## Support

If you have any questions, issues, or suggestions, please reach out to us on GitHub by [opening an issue](https://github.com/yourusername/asynmov/issues) or by contacting the maintainers directly through their GitHub profiles.

Happy historical world building with Asynmov!
