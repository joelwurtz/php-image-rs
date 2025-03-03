<?php

$start = microtime(true);
$source = __DIR__ . "/image.jpg";
$iterations = 10;
//$script = new \ImageScript();
//
//for ($i = 0; $i < $iterations; $i++) {
//    $script->transformFast($source, "result.webp");
//}
//
//echo "Resize with rust time (fast resize): " . (microtime(true) - $start) . PHP_EOL;
//
//$start = microtime(true);
//$script = new \ImageScript();
//
//for ($i = 0; $i < $iterations; $i++) {
//    $script->transform($source, "result2.webp");
//}
//
//echo "Resize with rust time: " . (microtime(true) - $start) . PHP_EOL;

$start = microtime(true);
$script = new \ImageScript();

for ($i = 0; $i < $iterations; $i++) {
    $script->transformWebp($source, "result3.webp");
}

echo "Resize with rust time, with webp sys: " . (microtime(true) - $start) . PHP_EOL;

// use imagick to resize image

$start = microtime(true);

for ($i = 0; $i < $iterations; $i++) {
    $image = new Imagick($source);
    $image->resizeImage(500, 500, Imagick::FILTER_LANCZOS, 0);
    $image->setOption('webp:near-lossless', 90);
    $image->setOption('webp:method', '6');
    $image->setOption('webp:hint', 'photo');
    $image->setOption('webp:low-memory', 1);
    $image->writeImage("result4.webp");
}

echo "Imagick Time: " . (microtime(true) - $start) . PHP_EOL;
