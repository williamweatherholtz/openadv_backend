# openadv_backend

OpenAdventure is a system for designing tabletop role playing game (TTRPG) systems and information in such that the any rule of tabletop games can be implemented.  This is done by using primitives to describe general game element traits, then combining these primitives into game-specific elements.

This project is intended to be implemented in a model-view-X architecture like Model-View-Presenter, Model-View-Adapter, Model-View-Controller, or Model-View-ViewModel, where actual data are housed in the model, user-interactive elements are handled by the view, and the remaining class connects data between the two.

By separating concerns in this way, we hope to make an adaptable, fast, small framework that can easily used to make connections with friends, regardless of their familiarity with the game.