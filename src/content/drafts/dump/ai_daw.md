+++
title = "ideas for an ai powered digital audio workstation"
date = 2020-09-07
draft = true
[extra]
categories = ["⛑️", "💡"]
+++

neural network powered daw
- PWA
- JS daw engine
- Tensorflow Pattern Generation Engine
- Choose Design Language (Material, )

*development in progress... i am currently working on this*

- **current state of (prototype) development:** 
  - the lyrics get generated with a method similar to [**rapwiz42**](https://colab.research.google.com/drive/1M27YXWJhepd4rhsB2KwRKGXZTwZRNIvz)
  - the lyrics get read out line by line by [**tacotron**](https://github.com/NVIDIA/tacotron2)
  - the sentences get stretched to fit the beat, concatenated and merged with the beat with a daw powered by 
  [**sox**](https://sox.sourceforge.net/) / [**pysox**](https://pypi.org/project/pysox/) 
- **dream features**
  - procedural / ai-powered beat generation (midi and/or sample based)
  - a *better* rap voice with a good balance between understandability and rhythm.
  - preserving the *dialogue mode* where config is queried one setting at a time in a dialogue (do you want a? (y/N) > y) 
  - preserving the *batch mode* where you can specify a song.yaml config file  
  - adding a *tui and/or gui mode*

---

## notes

- instruments: 
  - sampler: plays chopped samples triggered by midi notes
  - synthesizer: synthesizes sound triggered by midi notes
  - voice(ai): synthesizes a (rap) voice triggered by midi notes (flow) and with a separate lyrics track
- track:
  - a track is a collection of notes with a corresponding instrument
- pattern:
  - a pattern is a collection of tracks with a length in bars of a power of 2 (1, 2, 4, ... bars)
- playlist:
  - a playlist is a collection of patterns defining a song
