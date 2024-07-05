import { error, getInput, notice } from "@actions/core";
import { getOctokit, context } from "@actions/github";
import { readFile } from "fs/promises";
import { parse } from "toml";

async function run() {
    const token = getInput("token", { required: true });
    const path_cargo = getInput("toml_path", {
        required: false,
        trimWhitespace: true,
    });
    const filePath = getInput("file_path", { required: true, trimWhitespace: true });
    let version = "";
    try {
        const content = await readFile(path_cargo || "./Cargo.toml", "utf-8");
        const parsed = parse(content);
        if (parsed.package && parsed.package.version) {
            version = `v${parsed.package.version}`;
            console.log(`Version found in Cargo.toml: ${version}`);
        } else {
            error("No version found in Cargo.toml");
            return;
        }
    } catch (e) {
        error(`Error getting version from Cargo.toml: ${e}`);
        return;
    }
    const octokit = getOctokit(token);
    const repo_name = context.repo.repo;
    const repo_owner = context.repo.owner;
    const tags = await octokit.rest.repos.listTags({
        repo: repo_name,
        owner: repo_owner,
    });
    const tag = tags.data.find((tag) => tag.name === version);
    if (tag) {
        notice(`Tag already exists: ${tag.name}`);
        return;
    }
    const release = await octokit.rest.repos.createRelease({
        repo: repo_name,
        owner: repo_owner,
        tag_name: version,
    });
    await new Promise((resolve) => setTimeout(resolve, 2000));
    notice(`Release created: ${release.data.html_url}`);
    const fileBuffer = await readFile(filePath);
    release.data.upload_url
    const res = await octokit.rest.repos.uploadReleaseAsset({
        repo: repo_name,
        owner: repo_owner,
        release_id: release.data.id,
        name: "start_protected_game.exe",
        headers: {
            "content-type": "application/octet-stream",
            "content-length": fileBuffer.length,
        },
        data: fileBuffer as unknown as string,
    });

    notice(`Asset uploaded: ${res.data.browser_download_url}`);
}

run();
