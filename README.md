# replay-runner
Plugin to automatically run shared content replays for recording, requested by [PlayAid.app](https://twitter.com/PlayAidAI).

To use, populate the array in playaid.rs with the IDs you would like to view, and then build the plugin. Once built, open smash, press A at the title screen, and let go of your controller - the application will automatically play all replays in the array. It skips all bad replay ids to prevent hanging.

Special thanks to @RayTwo for his scene reverse engineering, one of the navigation pieces required this to recognize the menu progression.

Thanks to @jugeeya as well for his [results screen skip plugin](https://github.com/jugeeya/results-screen-skip/), this was used for inspiration on forcing menu navigation through the npad state functions.
