#!/usr/bin/env bash

cd -P "$(dirname ${0-$BASHSOURCE})/.." || exit 1

CURRENT_BRANCH="$(git branch --show-current)"
git push origin "${CURRENT_BRANCH}"
git push github "${CURRENT_BRANCH}"
git push gitea  "${CURRENT_BRANCH}"
