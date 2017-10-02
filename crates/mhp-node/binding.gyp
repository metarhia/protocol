{
  'variables': {
    'mhp_base_cflags': [
      '-Wall',
      '-Wextra',
      '-Wno-unused-parameter',
      '-std=c11',
    ],
    'mhp_debug_cflags': ['-g', '-O0'],
    'mhp_release_cflags': ['-O3'],
  },
  'targets': [
    {
      'target_name': 'mhp',
      'sources': ['src/binding.c'],
      'conditions': [
        ['OS == "win"', {
          'libraries': [
            '../../../target/release/mhp_node.lib',
          ],
        }, {
          'libraries': [
            '../../../target/release/libmhp_node.a',
          ],
        }],
      ],
      'configurations': {
        'Debug': {
          'cflags': ['<@(mhp_debug_cflags)'],
          'xcode_settings': {
            'OTHER_CFLAGS': ['<@(mhp_debug_cflags)'],
          },
        },
        'Release': {
          'cflags': ['<@(mhp_release_cflags)'],
          'xcode_settings': {
            'OTHER_CFLAGS': ['<@(mhp_release_cflags)'],
          },
        },
      },
      'cflags': ['<@(mhp_base_cflags)'],
      'xcode_settings': {
        'OTHER_CFLAGS': ['<@(mhp_base_cflags)'],
      },
    },
  ],
}
