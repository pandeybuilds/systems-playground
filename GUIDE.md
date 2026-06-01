# The Daily Routine (cheat-sheet)

You don't need to understand *why* these work yet. Just know the rhythm.
Everything below is typed in the **Terminal** app, one line at a time,
pressing Enter after each.

## Step 0 — go into my project folder (once per session)

```sh
cd ~/systems-playground
```

`cd` means "change directory" = "step into this folder." The `~` means your
home folder. So this says: "step into the systems-playground folder."

## The save-a-snapshot loop (the important part)

```sh
git status
```
Shows what has changed since the last snapshot. (Just looking — changes nothing.)

```sh
git add .
```
"Get everything ready to be saved." The `.` means "all the changes."

```sh
git commit -m "Day N: what I did today"
```
Take the snapshot and label it. The text in quotes is your note-to-future-self.

```sh
git push
```
Upload the snapshot to GitHub. This is what lights up the green square.

## That's the whole loop

```
git status   →   git add .   →   git commit -m "..."   →   git push
  (look)          (prepare)         (save snapshot)         (upload)
```

Do this once a day and you're winning.
